defmodule Katanaute.Repo do
  use Ecto.Repo,
    otp_app: :katanaute,
    adapter: Ecto.Adapters.SQLite3
end
