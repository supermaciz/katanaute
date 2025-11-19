defmodule KatanauteWeb.PageController do
  use KatanauteWeb, :controller

  def home(conn, _params) do
    render(conn, :home)
  end

  def react(conn, _params) do
    # Serve React's index.html for SPA client-side routing
    conn
    |> put_resp_content_type("text/html")
    |> send_file(200, Application.app_dir(:katanaute, "priv/static/react/index.html"))
  end
end
