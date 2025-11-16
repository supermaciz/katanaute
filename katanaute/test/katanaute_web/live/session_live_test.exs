defmodule KatanauteWeb.SessionLiveTest do
  use KatanauteWeb.ConnCase

  import Phoenix.LiveViewTest
  import Katanaute.TrainingFixtures
  import Katanaute.CurriculumFixtures

  @invalid_attrs %{practiced_at: nil, in_course: false, notes: nil, kata_id: nil}

  setup %{conn: conn} do
    # Create a test user
    user = Katanaute.AccountsFixtures.user_fixture()
    # Create unique katas for this test run
    kata = kata_fixture(%{name: "Test Kata #{System.unique_integer([:positive])}"})

    conn = log_in_user(conn, user)

    create_attrs = %{
      practiced_at: "2025-09-19T00:35:00Z",
      in_course: true,
      notes: "some notes",
      kata_id: kata.id
    }

    update_attrs = %{
      practiced_at: "2025-09-20T00:35:00Z",
      in_course: false,
      notes: "some updated notes",
      kata_id: kata.id
    }

    %{conn: conn, user: user, kata: kata, create_attrs: create_attrs, update_attrs: update_attrs}
  end

  defp create_session(%{conn: conn, create_attrs: create_attrs, user: user}) do
    # Extract kata_id from create_attrs but create session for the user
    kata_id = create_attrs.kata_id
    session = session_fixture(%{kata_id: kata_id, user_id: user.id})
    %{session: session, conn: conn}
  end

  describe "Index" do
    setup [:create_session]

    test "lists all sessions", %{conn: conn, session: session} do
      {:ok, _index_live, html} = live(conn, ~p"/sessions")

      assert html =~ "Listing Sessions"
      assert html =~ session.notes
    end

    test "saves new session", %{conn: conn, create_attrs: create_attrs, user: user} do
      # Load the form directly with the user_id parameter
      {:ok, form_live, _html} = live(conn, "/sessions/new?user_id=#{user.id}")

      assert render(form_live) =~ "New Session"

      assert form_live
             |> form("#session-form", session: @invalid_attrs)
             |> render_change() =~ "can&#39;t be blank"

      assert {:ok, index_live, _html} =
               form_live
               |> form("#session-form", session: create_attrs)
               |> render_submit()
               |> follow_redirect(conn, "/sessions")

      html = render(index_live)
      assert html =~ "Session created successfully"
      assert html =~ "some notes"
    end

    test "updates session in listing", %{conn: conn, session: session, update_attrs: update_attrs} do
      {:ok, index_live, _html} = live(conn, ~p"/sessions")

      assert {:ok, form_live, _html} =
               index_live
               |> element("#sessions-#{session.id} a", "Edit")
               |> render_click()
               |> follow_redirect(conn, ~p"/sessions/#{session}/edit")

      assert render(form_live) =~ "Edit Session"

      assert form_live
             |> form("#session-form", session: @invalid_attrs)
             |> render_change() =~ "can&#39;t be blank"

      assert {:ok, index_live, _html} =
               form_live
               |> form("#session-form", session: update_attrs)
               |> render_submit()
               |> follow_redirect(conn, ~p"/sessions")

      html = render(index_live)
      assert html =~ "Session updated successfully"
      assert html =~ "some updated notes"
    end

    test "deletes session in listing", %{conn: conn, session: session} do
      {:ok, index_live, _html} = live(conn, ~p"/sessions")

      assert index_live |> element("#sessions-#{session.id} a", "Delete") |> render_click()
      refute has_element?(index_live, "#sessions-#{session.id}")
    end
  end

  describe "Show" do
    setup [:create_session]

    test "displays session", %{conn: conn, session: session} do
      {:ok, _show_live, html} = live(conn, ~p"/sessions/#{session}")

      assert html =~ "Show Session"
      assert html =~ session.notes
    end

    test "updates session and returns to show", %{
      conn: conn,
      session: session,
      update_attrs: update_attrs
    } do
      {:ok, show_live, _html} = live(conn, ~p"/sessions/#{session}")

      assert {:ok, form_live, _} =
               show_live
               |> element("a", "Edit")
               |> render_click()
               |> follow_redirect(conn, ~p"/sessions/#{session}/edit?return_to=show")

      assert render(form_live) =~ "Edit Session"

      assert form_live
             |> form("#session-form", session: @invalid_attrs)
             |> render_change() =~ "can&#39;t be blank"

      assert {:ok, show_live, _html} =
               form_live
               |> form("#session-form", session: update_attrs)
               |> render_submit()
               |> follow_redirect(conn, ~p"/sessions/#{session}")

      html = render(show_live)
      assert html =~ "Session updated successfully"
      assert html =~ "some updated notes"
    end
  end
end
