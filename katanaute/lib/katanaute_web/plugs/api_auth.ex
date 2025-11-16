defmodule KatanauteWeb.Plugs.ApiAuth do
  @moduledoc """
  Plug for API bearer token authentication.

  This plug checks for a valid Authorization header with a bearer token
  and loads the associated user into the connection.
  """

  import Plug.Conn
  import Phoenix.Controller

  alias Katanaute.Accounts

  def init(opts), do: opts

  def call(conn, _opts) do
    case get_req_header(conn, "authorization") do
      ["Bearer " <> token] ->
        authenticate_user(conn, token)

      _ ->
        conn
    end
  end

  defp authenticate_user(conn, token) do
    case Accounts.get_user_by_api_token(token) do
      nil ->
        conn

      user ->
        conn
        |> assign(:current_user, user)
    end
  end

  @doc """
  Ensures the user is authenticated via API token.

  If not, returns a 401 Unauthorized response.
  """
  def require_authenticated_user(conn, _opts) do
    if Map.has_key?(conn.assigns, :current_user) do
      conn
    else
      conn
      |> put_status(:unauthorized)
      |> put_view(json: KatanauteWeb.ErrorJSON)
      |> render(:"401")
      |> halt()
    end
  end
end
