defmodule KatanauteWeb.SessionController do
  use KatanauteWeb, :controller

  alias Katanaute.Training
  alias Katanaute.Training.Session

  require Logger

  action_fallback KatanauteWeb.FallbackController

  def index(conn, _params) do
    session = Training.list_sessions()
    render(conn, :index, session: session)
  end

  def create(conn, %{"session" => session_params}) do
    Logger.info("Creating session with params: #{inspect(session_params)}")

    with {:ok, %Session{} = session} <- Training.create_session(session_params) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", ~p"/api/sessions/#{session}")
      |> render(:show, session: session)
    end
  end

  def show(conn, %{"id" => id}) do
    session = Training.get_session!(id)
    render(conn, :show, session: session)
  end

  def update(conn, %{"id" => id, "session" => session_params}) do
    session = Training.get_session!(id)

    with {:ok, %Session{} = session} <- Training.update_session(session, session_params) do
      render(conn, :show, session: session)
    end
  end

  def delete(conn, %{"id" => id}) do
    session = Training.get_session!(id)

    with {:ok, %Session{}} <- Training.delete_session(session) do
      send_resp(conn, :no_content, "")
    end
  end
end
