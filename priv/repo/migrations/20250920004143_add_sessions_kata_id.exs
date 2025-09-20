defmodule Katanaute.Repo.Migrations.AddSessionsKataId do
  use Ecto.Migration

  def change do
    alter table(:sessions) do
      add :kata_id, references(:katas, on_delete: :delete_all), null: false
    end
  end
end
