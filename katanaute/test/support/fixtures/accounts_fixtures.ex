defmodule Katanaute.AccountsFixtures do
  @moduledoc """
  Test helpers for creating users and API credentials.
  """

  alias Katanaute.Accounts

  def unique_user_email do
    "user" <> Integer.to_string(System.unique_integer([:positive])) <> "@example.com"
  end

  def valid_user_password do
    "verysecurepwd"
  end

  @doc """
  Generates a confirmed user.
  """
  def user_fixture(attrs \\ %{}) do
    {:ok, user} =
      attrs
      |> Enum.into(%{email: unique_user_email(), password: valid_user_password()})
      |> Accounts.register_user()

    user
  end

  @doc """
  Generates an API token for the given user, or creates one on the fly.
  Returns the token string for Authorization headers.
  """
  def api_token_fixture(user \\ user_fixture()) do
    Accounts.generate_user_api_token(user)
  end
end
