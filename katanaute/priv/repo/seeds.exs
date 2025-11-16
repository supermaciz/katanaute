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
alias Katanaute.Accounts

# Create katas
kata1 = Repo.insert!(%Kata{name: "Sanchin", level: :yellow})
kata2 = Repo.insert!(%Kata{name: "Kanshiwa", level: :green})
kata3 = Repo.insert!(%Kata{name: "Kanshu", level: :blue})
kata4 = Repo.insert!(%Kata{name: "Seichin", level: :brown})
kata5 = Repo.insert!(%Kata{name: "Seisan", level: :shodan})

# Create test users
{:ok, user1} =
  Accounts.register_user(%{
    email: "test@example.com",
    password: "testpassword123"
  })

{:ok, user2} =
  Accounts.register_user(%{
    email: "demo@example.com",
    password: "demopassword123"
  })

# Create sample sessions for test user
alias Katanaute.Training

Training.create_session(%{
  user_id: user1.id,
  kata_id: kata1.id,
  practiced_at: DateTime.utc_now(),
  in_course: true,
  notes: "First practice session with Sanchin kata. Focused on breathing and stance."
})

Training.create_session(%{
  user_id: user1.id,
  kata_id: kata1.id,
  practiced_at: DateTime.utc_now() |> DateTime.add(-86400, :second),
  in_course: true,
  notes: "Second session. Improving form and power."
})

Training.create_session(%{
  user_id: user2.id,
  kata_id: kata2.id,
  practiced_at: DateTime.utc_now(),
  in_course: false,
  notes: "Demo user practicing Kanshiwa kata."
})

IO.puts("\nSeeds completed!")
IO.puts("Test users created:")
IO.puts("  - test@example.com (password: testpassword123)")
IO.puts("  - demo@example.com (password: demopassword123)")
