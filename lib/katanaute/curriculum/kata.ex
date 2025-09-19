defmodule Katanaute.Curriculum.Kata do
  use Ecto.Schema
  import Ecto.Changeset

  schema "katas" do
    field :name, :string

    field :level, Ecto.Enum,
      values: [yellow: 1, orange: 2, green: 3, blue: 4, brown: 5, shodan: 6]

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(kata, attrs) do
    kata
    |> cast(attrs, [:name, :level])
    |> validate_required([:name, :level])
  end
end
