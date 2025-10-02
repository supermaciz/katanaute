defmodule KatanauteWeb.KataController do
  use KatanauteWeb, :controller

  alias Katanaute.Curriculum
  alias Katanaute.Curriculum.Kata

  action_fallback KatanauteWeb.FallbackController

  def index(conn, _params) do
    katas = Curriculum.list_katas()
    render(conn, :index, katas: katas)
  end

  def create(conn, %{"kata" => kata_params}) do
    with {:ok, %Kata{} = kata} <- Curriculum.create_kata(kata_params) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", ~p"/api/katas/#{kata}")
      |> render(:show, kata: kata)
    end
  end

  def show(conn, %{"id" => id}) do
    kata = Curriculum.get_kata!(id)
    render(conn, :show, kata: kata)
  end

  def update(conn, %{"id" => id, "kata" => kata_params}) do
    kata = Curriculum.get_kata!(id)

    with {:ok, %Kata{} = kata} <- Curriculum.update_kata(kata, kata_params) do
      render(conn, :show, kata: kata)
    end
  end

  def delete(conn, %{"id" => id}) do
    kata = Curriculum.get_kata!(id)

    with {:ok, %Kata{}} <- Curriculum.delete_kata(kata) do
      send_resp(conn, :no_content, "")
    end
  end
end
