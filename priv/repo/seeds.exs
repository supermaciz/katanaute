# Script for populating the database. You can run it as:
#
#     mix run priv/repo/seeds.exs
#
# Inside the script, you can read and write to any of your
# repositories directly:
#
#     Katanaute.Repo.insert!(%Katanaute.SomeSchema{})
#
# We recommend using the bang functions (`insert!`, `update!`
# and so on) as they will fail if something goes wrong.

alias Katanaute.Repo
alias Katanaute.Curriculum.Kata

Repo.insert!(%Kata{name: "Sanchin", level: :yellow})
Repo.insert!(%Kata{name: "Kanshiwa", level: :green})
Repo.insert!(%Kata{name: "Kanshu", level: :blue})
Repo.insert!(%Kata{name: "Seichin", level: :brown})
Repo.insert!(%Kata{name: "Seisan", level: :shodan})
