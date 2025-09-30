defmodule Katanaute.TrainingFixtures do
  @moduledoc """
  This module defines test helpers for creating
  entities via the `Katanaute.Training` context.
  """

  @doc """
  Generate a session.
  """
  def session_fixture(attrs \\ %{}) do
    {:ok, session} =
      attrs
      |> Enum.into(%{
        in_course: true,
        notes: "some notes",
        practiced_at: ~U[2025-09-19 00:35:00Z]
      })
      |> Katanaute.Training.create_session()

    session
  end
end
