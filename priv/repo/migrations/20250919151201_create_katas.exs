defmodule Katanaute.Repo.Migrations.CreateKatas do
  use Ecto.Migration

  def change do
    create table(:katas) do
      add :name, :string
      add :level, :integer

      timestamps(type: :utc_datetime)
    end
  end
end
