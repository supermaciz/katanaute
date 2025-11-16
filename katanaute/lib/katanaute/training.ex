defmodule Katanaute.Training do
  @moduledoc """
  The Training context.
  """

  import Ecto.Query, warn: false
  alias Katanaute.Repo

  alias Katanaute.Training.Session

  @doc """
  Returns the list of sessions.

  ## Examples

      iex> list_sessions()
      [%Session{}, ...]

  """
  def list_sessions do
    Repo.all(Session)
    |> Repo.preload(:kata)
  end

  @doc """
  Returns the list of sessions for a specific user.

  ## Examples

      iex> list_user_sessions(user_id)
      [%Session{}, ...]

  """
  def list_user_sessions(user_id) do
    from(s in Session, where: s.user_id == ^user_id)
    |> Repo.all()
    |> Repo.preload(:kata)
  end

  @doc """
  Gets a single session.

  Raises `Ecto.NoResultsError` if the Session does not exist.

  ## Examples

      iex> get_session!(123)
      %Session{}

      iex> get_session!(456)
      ** (Ecto.NoResultsError)

  """
  def get_session!(id) do
    Repo.get!(Session, id)
    |> Repo.preload(:kata)
  end

  @doc """
  Gets a single session for a specific user.

  Raises `Ecto.NoResultsError` if the Session does not exist or doesn't belong to the user.

  ## Examples

      iex> get_user_session!(user_id, session_id)
      %Session{}

      iex> get_user_session!(user_id, other_user_session_id)
      ** (Ecto.NoResultsError)

  """
  def get_user_session!(user_id, id) do
    from(s in Session, where: s.id == ^id and s.user_id == ^user_id)
    |> Repo.one!()
    |> Repo.preload(:kata)
  end

  @doc """
  Creates a session.

  ## Examples

      iex> create_session(%{field: value})
      {:ok, %Session{}}

      iex> create_session(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_session(attrs) do
    %Session{}
    |> Session.changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Updates a session.

  ## Examples

      iex> update_session(session, %{field: new_value})
      {:ok, %Session{}}

      iex> update_session(session, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_session(%Session{} = session, attrs) do
    session
    |> Session.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a session.

  ## Examples

      iex> delete_session(session)
      {:ok, %Session{}}

      iex> delete_session(session)
      {:error, %Ecto.Changeset{}}

  """
  def delete_session(%Session{} = session) do
    Repo.delete(session)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking session changes.

  ## Examples

      iex> change_session(session)
      %Ecto.Changeset{data: %Session{}}

  """
  def change_session(%Session{} = session, attrs \\ %{}) do
    Session.changeset(session, attrs)
  end
end
