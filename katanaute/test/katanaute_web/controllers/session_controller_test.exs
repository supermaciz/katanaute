defmodule KatanauteWeb.SessionControllerTest do
  use KatanauteWeb.ConnCase

  import Katanaute.TrainingFixtures
  alias Katanaute.Training.Session

  @create_attrs %{
    practiced_at: ~U[2025-10-01 12:34:00Z],
    in_course: true,
    notes: "some notes",
    kata_id: 42
  }
  @update_attrs %{
    practiced_at: ~U[2025-10-02 12:34:00Z],
    in_course: false,
    notes: "some updated notes",
    kata_id: 43
  }
  @invalid_attrs %{practiced_at: nil, in_course: nil, notes: nil, kata_id: nil}

  setup %{conn: conn} do
    {:ok, conn: put_req_header(conn, "accept", "application/json")}
  end

  describe "index" do
    test "lists all session", %{conn: conn} do
      conn = get(conn, ~p"/api/session")
      assert json_response(conn, 200)["data"] == []
    end
  end

  describe "create session" do
    test "renders session when data is valid", %{conn: conn} do
      conn = post(conn, ~p"/api/session", session: @create_attrs)
      assert %{"id" => id} = json_response(conn, 201)["data"]

      conn = get(conn, ~p"/api/session/#{id}")

      assert %{
               "id" => ^id,
               "in_course" => true,
               "kata_id" => 42,
               "notes" => "some notes",
               "practiced_at" => "2025-10-01T12:34:00Z"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn} do
      conn = post(conn, ~p"/api/session", session: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "update session" do
    setup [:create_session]

    test "renders session when data is valid", %{conn: conn, session: %Session{id: id} = session} do
      conn = put(conn, ~p"/api/session/#{session}", session: @update_attrs)
      assert %{"id" => ^id} = json_response(conn, 200)["data"]

      conn = get(conn, ~p"/api/session/#{id}")

      assert %{
               "id" => ^id,
               "in_course" => false,
               "kata_id" => 43,
               "notes" => "some updated notes",
               "practiced_at" => "2025-10-02T12:34:00Z"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn, session: session} do
      conn = put(conn, ~p"/api/session/#{session}", session: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "delete session" do
    setup [:create_session]

    test "deletes chosen session", %{conn: conn, session: session} do
      conn = delete(conn, ~p"/api/session/#{session}")
      assert response(conn, 204)

      assert_error_sent 404, fn ->
        get(conn, ~p"/api/session/#{session}")
      end
    end
  end

  defp create_session(_) do
    session = session_fixture()

    %{session: session}
  end
end
