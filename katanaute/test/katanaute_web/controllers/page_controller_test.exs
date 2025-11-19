defmodule KatanauteWeb.PageControllerTest do
  use KatanauteWeb.ConnCase

  test "GET /admin (Phoenix home page)", %{conn: conn} do
    conn = get(conn, ~p"/admin")
    assert html_response(conn, 200) =~ "Peace of mind from prototype to production"
  end

  describe "React SPA" do
    @tag :skip
    test "GET / serves React index.html", %{conn: conn} do
      # Skipped: Requires React build in priv/static/react/index.html
      # This is tested in integration tests after React is built
      conn = get(conn, ~p"/")
      assert html_response(conn, 200)
    end
  end
end
