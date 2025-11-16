defmodule Katanaute.Accounts.DeviceCode do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  # Device codes expire in 15 minutes
  @device_code_validity_in_minutes 15

  schema "device_codes" do
    field :device_code, :string
    field :user_code, :string
    field :status, :string, default: "pending"
    field :expires_at, :utc_datetime
    belongs_to :user, Katanaute.Accounts.User

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(device_code, attrs) do
    device_code
    |> cast(attrs, [:device_code, :user_code, :user_id, :status, :expires_at])
    |> validate_required([:device_code, :user_code, :status, :expires_at])
    |> validate_inclusion(:status, ["pending", "approved", "expired", "denied"])
    |> unique_constraint(:device_code)
    |> unique_constraint(:user_code)
  end

  @doc """
  Generates a new device code and user code.
  """
  def build_device_code do
    device_code = generate_device_code()
    user_code = generate_user_code()
    expires_at = DateTime.utc_now() |> DateTime.add(@device_code_validity_in_minutes * 60, :second) |> DateTime.truncate(:second)

    %__MODULE__{
      device_code: device_code,
      user_code: user_code,
      status: "pending",
      expires_at: expires_at
    }
  end

  # Generates a random device code (UUID-like).
  defp generate_device_code do
    :crypto.strong_rand_bytes(32)
    |> Base.url_encode64(padding: false)
  end

  # Generates a human-readable user code (e.g., "ABCD-1234").
  defp generate_user_code do
    # Generate 8 random alphanumeric characters
    part1 = generate_code_part(4)
    part2 = generate_code_part(4)
    "#{part1}-#{part2}"
  end

  defp generate_code_part(length) do
    # Use uppercase letters and digits, excluding confusing characters (0, O, 1, I, L)
    charset = "ABCDEFGHJKMNPQRSTUVWXYZ23456789"
    charset_size = String.length(charset)

    for _ <- 1..length, into: "" do
      index = :rand.uniform(charset_size) - 1
      String.at(charset, index)
    end
  end

  @doc """
  Query to find a device code by device_code value.
  """
  def by_device_code_query(device_code) do
    from d in __MODULE__,
      where: d.device_code == ^device_code
  end

  @doc """
  Query to find a device code by user_code value.
  """
  def by_user_code_query(user_code) do
    from d in __MODULE__,
      where: d.user_code == ^user_code
  end

  @doc """
  Query to find valid (non-expired) device codes.
  """
  def valid_device_codes_query do
    now = DateTime.utc_now()

    from d in __MODULE__,
      where: d.expires_at > ^now
  end

  @doc """
  Updates the device code status.
  """
  def approve_changeset(device_code, user_id) do
    device_code
    |> change(status: "approved", user_id: user_id)
  end

  def deny_changeset(device_code) do
    device_code
    |> change(status: "denied")
  end
end
