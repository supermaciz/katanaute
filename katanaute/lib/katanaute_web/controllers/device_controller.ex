defmodule KatanauteWeb.DeviceController do
  use KatanauteWeb, :controller

  alias Katanaute.Accounts

  @doc """
  GET /device
  Show the device authorization page where users enter their code.
  """
  def new(conn, params) do
    render(conn, :new, user_code: params["user_code"], error: nil)
  end

  @doc """
  POST /device
  Verify the user code and show authorization confirmation.
  """
  def verify(conn, %{"user_code" => user_code}) do
    case Accounts.get_device_code_by_user_code(String.upcase(user_code)) do
      nil ->
        render(conn, :new, user_code: user_code, error: "Invalid or expired code")

      device_code ->
        if conn.assigns[:current_user] do
          # User is logged in, show approval page
          render(conn, :approve, device_code: device_code)
        else
          # User needs to log in first
          conn
          |> put_session(:device_code_id, device_code.id)
          |> put_session(:user_return_to, ~p"/device/authorize")
          |> put_flash(:info, "Please log in to authorize this device")
          |> redirect(to: ~p"/users/log_in")
        end
    end
  end

  def verify(conn, %{"device" => %{"user_code" => user_code}}) do
    # Handle form submission with nested parameters
    verify(conn, %{"user_code" => user_code})
  end

  def verify(conn, _params) do
    render(conn, :new, user_code: "", error: "User code is required")
  end

  @doc """
  GET /device/authorize
  Show the authorization page after login (from redirect).
  """
  def authorize(conn, _params) do
    device_code_id = get_session(conn, :device_code_id)

    case device_code_id do
      nil ->
        conn
        |> put_flash(:error, "No device code found. Please enter the code again.")
        |> redirect(to: ~p"/device")

      id ->
        case Katanaute.Repo.get(Katanaute.Accounts.DeviceCode, id) do
          nil ->
            conn
            |> delete_session(:device_code_id)
            |> put_flash(:error, "Device code has expired. Please try again.")
            |> redirect(to: ~p"/device")

          device_code ->
            render(conn, :approve, device_code: device_code)
        end
    end
  end

  @doc """
  POST /device/approve
  Approve the device authorization.
  """
  def approve(conn, %{"device_code_id" => device_code_id}) do
    device_code = Katanaute.Repo.get!(Katanaute.Accounts.DeviceCode, device_code_id)
    user = conn.assigns.current_user

    case Accounts.approve_device_code(device_code, user) do
      {:ok, _device_code} ->
        conn
        |> delete_session(:device_code_id)
        |> put_flash(:info, "Device authorized successfully!")
        |> render(:success)

      {:error, _changeset} ->
        conn
        |> put_flash(:error, "Failed to authorize device")
        |> render(:approve, device_code: device_code)
    end
  end

  def approve(conn, %{"device" => %{"device_code_id" => device_code_id}}) do
    approve(conn, %{"device_code_id" => device_code_id})
  end

  @doc """
  POST /device/deny
  Deny the device authorization.
  """
  def deny(conn, %{"device_code_id" => device_code_id}) do
    device_code = Katanaute.Repo.get!(Katanaute.Accounts.DeviceCode, device_code_id)

    case Accounts.deny_device_code(device_code) do
      {:ok, _device_code} ->
        conn
        |> delete_session(:device_code_id)
        |> put_flash(:info, "Device authorization denied")
        |> redirect(to: ~p"/")

      {:error, _changeset} ->
        conn
        |> put_flash(:error, "Failed to deny device")
        |> render(:approve, device_code: device_code)
    end
  end

  def deny(conn, %{"device" => %{"device_code_id" => device_code_id}}) do
    deny(conn, %{"device_code_id" => device_code_id})
  end
end
