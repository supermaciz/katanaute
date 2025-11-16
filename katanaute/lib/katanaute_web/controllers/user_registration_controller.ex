defmodule KatanauteWeb.UserRegistrationController do
  use KatanauteWeb, :controller

  alias Katanaute.Accounts
  alias Katanaute.Accounts.User
  alias KatanauteWeb.Plugs.WebAuth

  def new(conn, _params) do
    changeset = Accounts.change_user_registration(%User{})
    render(conn, :new, changeset: changeset)
  end

  def create(conn, %{"user" => user_params}) do
    case Accounts.register_user(user_params) do
      {:ok, user} ->
        conn
        |> put_flash(:info, "User created successfully!")
        |> WebAuth.log_in_user(user)

      {:error, %Ecto.Changeset{} = changeset} ->
        render(conn, :new, changeset: changeset)
    end
  end
end
