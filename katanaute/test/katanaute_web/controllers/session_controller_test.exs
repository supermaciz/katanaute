defmodule KatanauteWeb.SessionControllerTest do
  use KatanauteWeb.ConnCase

  import Katanaute.CurriculumFixtures
  alias Katanaute.Training.Session

  setup do
    kata1 = kata_fixture(%{name: "Kata 1"})
    kata2 = kata_fixture(%{name: "Kata 2"})

    {:ok,
     create_attrs: %{
       practiced_at: "2025-10-01T12:34:00Z",
       in_course: true,
       notes: "some notes",
       kata_id: kata1.id
     },
     update_attrs: %{
       practiced_at: "2025-10-02T12:34:00Z",
       in_course: false,
       notes: "some updated notes",
       kata_id: kata2.id
     },
     invalid_attrs: %{practiced_at: nil, in_course: nil, notes: nil, kata_id: nil}}
  end

  setup %{conn: conn} do
    conn = put_req_header(conn, "accept", "application/json")
    {conn, user} = with_api_auth(conn)

    {:ok, conn: conn, user: user}
  end

  describe "index" do
    test "lists all session", %{conn: conn} do
      conn = get(conn, ~p"/api/sessions")
      assert json_response(conn, 200)["data"] == []
    end
  end

  describe "create session" do
    test "renders session when data is valid", %{conn: conn, create_attrs: create_attrs} do
      kata_id = create_attrs.kata_id
      conn = post(conn, ~p"/api/sessions", session: create_attrs)
      assert %{"id" => id} = json_response(conn, 201)["data"]

      conn = get(conn, ~p"/api/sessions/#{id}")

      assert %{
               "id" => ^id,
               "in_course" => true,
               "kata_id" => ^kata_id,
               "notes" => "some notes",
               "practiced_at" => "2025-10-01T12:34:00Z"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn, invalid_attrs: invalid_attrs} do
      conn = post(conn, ~p"/api/sessions", session: invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "update session" do
    setup [:create_session]

    test "renders session when data is valid", %{
      conn: conn,
      session: %Session{id: id} = session,
      update_attrs: update_attrs
    } do
      kata_id = update_attrs.kata_id
      conn = put(conn, ~p"/api/sessions/#{session}", session: update_attrs)
      assert %{"id" => ^id} = json_response(conn, 200)["data"]

      conn = get(conn, ~p"/api/sessions/#{id}")

      assert %{
               "id" => ^id,
               "in_course" => false,
               "kata_id" => ^kata_id,
               "notes" => "some updated notes",
               "practiced_at" => "2025-10-02T12:34:00Z"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{
      conn: conn,
      session: session,
      invalid_attrs: invalid_attrs
    } do
      conn = put(conn, ~p"/api/sessions/#{session}", session: invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "delete session" do
    setup [:create_session]

    test "deletes chosen session", %{conn: conn, session: session} do
      conn = delete(conn, ~p"/api/sessions/#{session}")
      assert response(conn, 204)

      assert_error_sent 404, fn ->
        get(conn, ~p"/api/sessions/#{session}")
      end
    end
  end

  defp create_session(%{create_attrs: create_attrs, user: user}) do
    session_attrs = Map.put(create_attrs, :user_id, user.id)
    {:ok, session} = Katanaute.Training.create_session(session_attrs)

    %{session: session}
  end
end
