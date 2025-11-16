defmodule Katanaute.Training.Session do
  use Ecto.Schema
  import Ecto.Changeset
  alias Katanaute.Curriculum.Kata

  schema "sessions" do
    field :practiced_at, :utc_datetime
    field :in_course, :boolean, default: false
    field :notes, :string
    belongs_to :kata, Kata
    belongs_to :user, Katanaute.Accounts.User

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(session, attrs) do
    session
    |> cast(attrs, [:practiced_at, :in_course, :notes, :kata_id, :user_id])
    |> validate_required([:practiced_at, :in_course, :kata_id, :user_id])
    |> foreign_key_constraint(:kata_id)
    |> foreign_key_constraint(:user_id)
  end
end
