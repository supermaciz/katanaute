defmodule KatanauteWeb.PageController do
  use KatanauteWeb, :controller

  def home(conn, _params) do
    render(conn, :home)
  end
end
