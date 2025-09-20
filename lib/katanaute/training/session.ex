defmodule Katanaute.Training.Session do
  use Ecto.Schema
  import Ecto.Changeset

  schema "sessions" do
    field :practiced_at, :utc_datetime
    field :in_course, :boolean, default: false
    field :notes, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(session, attrs) do
    session
    |> cast(attrs, [:practiced_at, :in_course, :notes])
    |> validate_required([:practiced_at, :in_course, :notes])
  end
end
