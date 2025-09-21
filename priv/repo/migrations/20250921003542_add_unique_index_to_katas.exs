defmodule Katanaute.Repo.Migrations.AddUniqueIndexToKatas do
  use Ecto.Migration

  def change do
    create unique_index("katas", [:name])
  end
end
