defmodule Katanaute.CurriculumFixtures do
  @moduledoc """
  This module defines test helpers for creating
  entities via the `Katanaute.Curriculum` context.
  """

  @doc """
  Generate a kata.
  """
  def kata_fixture(attrs \\ %{}) do
    {:ok, kata} =
      attrs
      |> Enum.into(%{
        level: :yellow,
        name: "some name"
      })
      |> Katanaute.Curriculum.create_kata()

    kata
  end
end
