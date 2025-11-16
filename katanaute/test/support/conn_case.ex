defmodule KatanauteWeb.ConnCase do
  @moduledoc """
  This module defines the test case to be used by
  tests that require setting up a connection.

  Such tests rely on `Phoenix.ConnTest` and also
  import other functionality to make it easier
  to build common data structures and query the data layer.

  Finally, if the test case interacts with the database,
  we enable the SQL sandbox, so changes done to the database
  are reverted at the end of every test. If you are using
  PostgreSQL, you can even run database tests asynchronously
  by setting `use KatanauteWeb.ConnCase, async: true`, although
  this option is not recommended for other databases.
  """

  use ExUnit.CaseTemplate

  using do
    quote do
      # The default endpoint for testing
      @endpoint KatanauteWeb.Endpoint

      use KatanauteWeb, :verified_routes

      # Import conveniences for testing with connections
      import Plug.Conn
      import Phoenix.ConnTest
      import KatanauteWeb.ConnCase
    end
  end

  setup tags do
    Katanaute.DataCase.setup_sandbox(tags)
    {:ok, conn: Phoenix.ConnTest.build_conn()}
  end

  @doc """
  Set up an authenticated connection with a user and API token.
  Returns {conn, user} tuple.
  """
  def with_api_auth(conn) do
    user = Katanaute.AccountsFixtures.user_fixture()
    token = Katanaute.AccountsFixtures.api_token_fixture(user)

    auth_conn = Plug.Conn.put_req_header(conn, "authorization", "Bearer #{token}")
    {auth_conn, user}
  end

  @doc """
  Set up an authenticated LiveView session with a logged-in user.
  Returns a map with the user that can be used as context in live tests.
  """
  def login_user(_context) do
    user = Katanaute.AccountsFixtures.user_fixture()
    token = Katanaute.Accounts.generate_user_session_token(user)

    %{user: user, user_token: token}
  end

  @doc """
  Logs the given user into the provided connection for browser tests.
  """
  def log_in_user(conn, user) do
    token = Katanaute.Accounts.generate_user_session_token(user)

    conn
    |> Phoenix.ConnTest.init_test_session(%{})
    |> Plug.Conn.put_session(:user_token, token)
  end
end
