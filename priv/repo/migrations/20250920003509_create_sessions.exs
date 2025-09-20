defmodule Katanaute.Repo.Migrations.CreateSessions do
  use Ecto.Migration

  def change do
    create table(:sessions) do
      add :practiced_at, :utc_datetime
      add :in_course, :boolean, default: false, null: false
      add :notes, :string

      timestamps(type: :utc_datetime)
    end
  end
end
