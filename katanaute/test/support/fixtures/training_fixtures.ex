defmodule Katanaute.TrainingFixtures do
  @moduledoc """
  This module defines test helpers for creating
  entities via the `Katanaute.Training` context.
  """

  alias Katanaute.CurriculumFixtures
  alias Katanaute.AccountsFixtures

  @doc """
  Generate a session.
  """
  def session_fixture(attrs \\ %{}) do
    kata = CurriculumFixtures.kata_fixture()
    user = AccountsFixtures.user_fixture()

    {:ok, session} =
      attrs
      |> Enum.into(%{
        in_course: true,
        notes: "some notes",
        practiced_at: ~U[2025-09-19 00:35:00Z],
        kata_id: kata.id,
        user_id: user.id
      })
      |> Katanaute.Training.create_session()

    session
  end
end
