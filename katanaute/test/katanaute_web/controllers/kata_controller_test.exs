defmodule KatanauteWeb.KataControllerTest do
  use KatanauteWeb.ConnCase

  import Katanaute.CurriculumFixtures
  alias Katanaute.Curriculum.Kata

  @create_attrs %{
    name: "some name",
    level: :yellow
  }
  @update_attrs %{
    name: "some updated name",
    level: :orange
  }
  @invalid_attrs %{name: nil, level: nil}

  setup %{conn: conn} do
    {:ok, conn: put_req_header(conn, "accept", "application/json")}
  end

  describe "index" do
    test "lists all katas", %{conn: conn} do
      conn = get(conn, ~p"/api/katas")
      assert json_response(conn, 200)["data"] == []
    end
  end

  describe "create kata" do
    test "renders kata when data is valid", %{conn: conn} do
      conn = post(conn, ~p"/api/katas", kata: @create_attrs)
      assert %{"id" => id} = json_response(conn, 201)["data"]

      conn = get(conn, ~p"/api/katas/#{id}")

      assert %{
               "id" => ^id,
               "level" => "yellow",
               "name" => "some name"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn} do
      conn = post(conn, ~p"/api/katas", kata: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "update kata" do
    setup [:create_kata]

    test "renders kata when data is valid", %{conn: conn, kata: %Kata{id: id} = kata} do
      conn = put(conn, ~p"/api/katas/#{kata}", kata: @update_attrs)
      assert %{"id" => ^id} = json_response(conn, 200)["data"]

      conn = get(conn, ~p"/api/katas/#{id}")

      assert %{
               "id" => ^id,
               "level" => "orange",
               "name" => "some updated name"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn, kata: kata} do
      conn = put(conn, ~p"/api/katas/#{kata}", kata: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "delete kata" do
    setup [:create_kata]

    test "deletes chosen kata", %{conn: conn, kata: kata} do
      conn = delete(conn, ~p"/api/katas/#{kata}")
      assert response(conn, 204)

      assert_error_sent 404, fn ->
        get(conn, ~p"/api/katas/#{kata}")
      end
    end
  end

  defp create_kata(_) do
    kata = kata_fixture()

    %{kata: kata}
  end
end
