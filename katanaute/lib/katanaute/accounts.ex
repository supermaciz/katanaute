defmodule Katanaute.Accounts do
  @moduledoc """
  The Accounts context.
  """

  import Ecto.Query, warn: false
  alias Katanaute.Repo

  alias Katanaute.Accounts.{User, UserToken, DeviceCode}

  ## Database getters

  @doc """
  Gets a user by email.

  ## Examples

      iex> get_user_by_email("foo@example.com")
      %User{}

      iex> get_user_by_email("unknown@example.com")
      nil

  """
  def get_user_by_email(email) when is_binary(email) do
    Repo.get_by(User, email: email)
  end

  @doc """
  Gets a user by email and password.

  ## Examples

      iex> get_user_by_email_and_password("foo@example.com", "correct_password")
      %User{}

      iex> get_user_by_email_and_password("foo@example.com", "invalid_password")
      nil

  """
  def get_user_by_email_and_password(email, password)
      when is_binary(email) and is_binary(password) do
    user = Repo.get_by(User, email: email)
    if User.valid_password?(user, password), do: user, else: nil
  end

  @doc """
  Gets a single user.

  Raises `Ecto.NoResultsError` if the User does not exist.

  ## Examples

      iex> get_user!(123)
      %User{}

      iex> get_user!(456)
      ** (Ecto.NoResultsError)

  """
  def get_user!(id), do: Repo.get!(User, id)

  ## User registration

  @doc """
  Registers a user.

  ## Examples

      iex> register_user(%{field: value})
      {:ok, %User{}}

      iex> register_user(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def register_user(attrs) do
    %User{}
    |> User.registration_changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking user changes.

  ## Examples

      iex> change_user_registration(user)
      %Ecto.Changeset{data: %User{}}

  """
  def change_user_registration(%User{} = user, attrs \\ %{}) do
    User.registration_changeset(user, attrs, hash_password: false, validate_email: false)
  end

  ## Settings

  @doc """
  Returns an `%Ecto.Changeset{}` for changing the user email.

  ## Examples

      iex> change_user_email(user)
      %Ecto.Changeset{data: %User{}}

  """
  def change_user_email(user, attrs \\ %{}) do
    User.email_changeset(user, attrs, validate_email: false)
  end

  @doc """
  Updates the user email using the given token.

  If the token matches, the user email is updated and the token is deleted.
  """
  def update_user_email(user, password, attrs) do
    changeset =
      user
      |> User.email_changeset(attrs)
      |> User.validate_current_password(password)

    Ecto.Multi.new()
    |> Ecto.Multi.update(:user, changeset)
    |> Ecto.Multi.delete_all(:tokens, UserToken.by_user_and_contexts_query(user, :all))
    |> Repo.transaction()
    |> case do
      {:ok, %{user: user}} -> {:ok, user}
      {:error, :user, changeset, _} -> {:error, changeset}
    end
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for changing the user password.

  ## Examples

      iex> change_user_password(user)
      %Ecto.Changeset{data: %User{}}

  """
  def change_user_password(user, attrs \\ %{}) do
    User.password_changeset(user, attrs, hash_password: false)
  end

  @doc """
  Updates the user password.

  ## Examples

      iex> update_user_password(user, "valid password", %{password: ...})
      {:ok, %User{}}

      iex> update_user_password(user, "invalid password", %{password: ...})
      {:error, %Ecto.Changeset{}}

  """
  def update_user_password(user, password, attrs) do
    changeset =
      user
      |> User.password_changeset(attrs)
      |> User.validate_current_password(password)

    Ecto.Multi.new()
    |> Ecto.Multi.update(:user, changeset)
    |> Ecto.Multi.delete_all(:tokens, UserToken.by_user_and_contexts_query(user, :all))
    |> Repo.transaction()
    |> case do
      {:ok, %{user: user}} -> {:ok, user}
      {:error, :user, changeset, _} -> {:error, changeset}
    end
  end

  ## Session tokens

  @doc """
  Generates a session token.
  """
  def generate_user_session_token(user) do
    {token, user_token} = UserToken.build_session_token(user)
    Repo.insert!(user_token)
    token
  end

  @doc """
  Gets the user with the given signed token.
  """
  def get_user_by_session_token(token) do
    {:ok, query} = UserToken.verify_session_token_query(token)
    Repo.one(query)
  end

  @doc """
  Deletes the signed token with the given context.
  """
  def delete_user_session_token(token) do
    Repo.delete_all(UserToken.by_token_and_context_query(token, "session"))
    :ok
  end

  ## API tokens

  @doc """
  Generates an API token for the user.

  The token returned must be saved somewhere safe as it cannot be retrieved again.
  """
  def generate_user_api_token(user) do
    {encoded_token, user_token} = UserToken.build_api_token(user)
    Repo.insert!(user_token)
    encoded_token
  end

  @doc """
  Gets the user by API token.

  The token must be the encoded token returned by `generate_user_api_token/1`.
  """
  def get_user_by_api_token(token) do
    with {:ok, decoded_token} <- Base.url_decode64(token, padding: false),
         hashed_token <- :crypto.hash(:sha256, decoded_token),
         {:ok, query} <- UserToken.verify_api_token_query(hashed_token) do
      Repo.one(query)
    else
      _ -> nil
    end
  end

  @doc """
  Deletes an API token.
  """
  def delete_user_api_token(token) do
    with {:ok, decoded_token} <- Base.url_decode64(token, padding: false),
         hashed_token <- :crypto.hash(:sha256, decoded_token) do
      Repo.delete_all(UserToken.by_token_and_context_query(hashed_token, "api"))
      :ok
    else
      _ -> :ok
    end
  end

  @doc """
  Lists all API tokens for a user.
  """
  def list_user_api_tokens(user) do
    Repo.all(UserToken.by_user_and_contexts_query(user, ["api"]))
  end

  ## Device Flow

  @doc """
  Creates a new device code for the OAuth device flow.

  Returns the device code struct with a device_code and user_code.
  """
  def create_device_code do
    device_code = DeviceCode.build_device_code()
    Repo.insert(device_code)
  end

  @doc """
  Gets a device code by its device_code value.
  """
  def get_device_code_by_code(device_code) do
    now = DateTime.utc_now()

    Repo.one(
      from d in DeviceCode,
        where: d.device_code == ^device_code and d.expires_at > ^now
    )
  end

  @doc """
  Gets a device code by its user_code value.
  """
  def get_device_code_by_user_code(user_code) do
    now = DateTime.utc_now()

    Repo.one(
      from d in DeviceCode,
        where: d.user_code == ^user_code and d.expires_at > ^now
    )
  end

  @doc """
  Approves a device code for the given user.
  """
  def approve_device_code(device_code, user) do
    device_code
    |> DeviceCode.approve_changeset(user.id)
    |> Repo.update()
  end

  @doc """
  Denies a device code.
  """
  def deny_device_code(device_code) do
    device_code
    |> DeviceCode.deny_changeset()
    |> Repo.update()
  end
end
