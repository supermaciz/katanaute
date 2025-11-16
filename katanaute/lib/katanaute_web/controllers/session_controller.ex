defmodule KatanauteWeb.SessionController do
  use KatanauteWeb, :controller

  alias Katanaute.Training
  alias Katanaute.Training.Session

  require Logger

  action_fallback KatanauteWeb.FallbackController

  def index(conn, _params) do
    user = conn.assigns.current_user
    session = Training.list_user_sessions(user.id)
    render(conn, :index, session: session)
  end

  def create(conn, %{"session" => session_params}) do
    user = conn.assigns.current_user
    Logger.info("Creating session with params: #{inspect(session_params)}")

    # Add user_id to the session params
    session_params = Map.put(session_params, "user_id", user.id)

    with {:ok, %Session{} = session} <- Training.create_session(session_params) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", ~p"/api/sessions/#{session}")
      |> render(:show, session: session)
    end
  end

  def show(conn, %{"id" => id}) do
    user = conn.assigns.current_user
    session = Training.get_user_session!(user.id, id)
    render(conn, :show, session: session)
  end

  def update(conn, %{"id" => id, "session" => session_params}) do
    user = conn.assigns.current_user
    session = Training.get_user_session!(user.id, id)

    with {:ok, %Session{} = session} <- Training.update_session(session, session_params) do
      render(conn, :show, session: session)
    end
  end

  def delete(conn, %{"id" => id}) do
    user = conn.assigns.current_user
    session = Training.get_user_session!(user.id, id)

    with {:ok, %Session{}} <- Training.delete_session(session) do
      send_resp(conn, :no_content, "")
    end
  end
end
