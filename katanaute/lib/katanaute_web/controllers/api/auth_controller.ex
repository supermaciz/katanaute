defmodule KatanauteWeb.API.AuthController do
  use KatanauteWeb, :controller

  alias Katanaute.Accounts

  action_fallback KatanauteWeb.FallbackController

  @doc """
  POST /api/auth/register
  Register a new user and return an API token.
  """
  def register(conn, %{"email" => email, "password" => password}) do
    case Accounts.register_user(%{email: email, password: password}) do
      {:ok, user} ->
        token = Accounts.generate_user_api_token(user)

        conn
        |> put_status(:created)
        |> json(%{
          data: %{
            access_token: token,
            token_type: "Bearer",
            user: %{id: user.id, email: user.email}
          }
        })

      {:error, changeset} ->
        conn
        |> put_status(:unprocessable_entity)
        |> json(%{errors: translate_errors(changeset)})
    end
  end

  @doc """
  POST /api/auth/token
  Login with email and password, return an API token.
  """
  def create_token(conn, %{"email" => email, "password" => password}) do
    case Accounts.get_user_by_email_and_password(email, password) do
      nil ->
        conn
        |> put_status(:unauthorized)
        |> json(%{error: "Invalid email or password"})

      user ->
        token = Accounts.generate_user_api_token(user)

        json(conn, %{
          data: %{
            access_token: token,
            token_type: "Bearer",
            user: %{id: user.id, email: user.email}
          }
        })
    end
  end

  @doc """
  DELETE /api/auth/token
  Revoke the current API token (logout).
  """
  def delete_token(conn, _params) do
    case get_req_header(conn, "authorization") do
      ["Bearer " <> token] ->
        Accounts.delete_user_api_token(token)

        conn
        |> put_status(:no_content)
        |> json(%{})

      _ ->
        conn
        |> put_status(:unauthorized)
        |> json(%{error: "No authorization token provided"})
    end
  end

  @doc """
  GET /api/auth/me
  Get current user information.
  Requires authentication.
  """
  def me(conn, _params) do
    user = conn.assigns.current_user

    json(conn, %{
      data: %{
        id: user.id,
        email: user.email,
        confirmed_at: user.confirmed_at
      }
    })
  end

  @doc """
  POST /api/auth/device/code
  Initiate device authorization flow.
  Returns device_code and user_code for the client.
  """
  def device_code(conn, _params) do
    case Accounts.create_device_code() do
      {:ok, device_code} ->
        # Build the verification URI
        verification_uri = url(~p"/admin/device")

        json(conn, %{
          data: %{
            device_code: device_code.device_code,
            user_code: device_code.user_code,
            verification_uri: verification_uri,
            verification_uri_complete: "#{verification_uri}?user_code=#{device_code.user_code}",
            expires_in: 900,
            # 15 minutes in seconds
            interval: 5
            # Poll every 5 seconds
          }
        })

      {:error, changeset} ->
        conn
        |> put_status(:unprocessable_entity)
        |> json(%{errors: translate_errors(changeset)})
    end
  end

  @doc """
  POST /api/auth/device/token
  Poll for device authorization completion.
  Returns an API token if the user has approved the device.
  """
  def device_token(conn, %{"device_code" => device_code_value}) do
    case Accounts.get_device_code_by_code(device_code_value) do
      nil ->
        conn
        |> put_status(:bad_request)
        |> json(%{error: "expired_token", error_description: "The device code has expired"})

      device_code ->
        handle_device_code_status(conn, device_code)
    end
  end

  def device_token(conn, _params) do
    conn
    |> put_status(:bad_request)
    |> json(%{error: "invalid_request", error_description: "device_code is required"})
  end

  defp handle_device_code_status(conn, %{status: "pending"}) do
    conn
    |> put_status(:ok)
    |> json(%{
      error: "authorization_pending",
      error_description: "The authorization request is still pending"
    })
  end

  defp handle_device_code_status(conn, %{status: "denied"}) do
    conn
    |> put_status(:forbidden)
    |> json(%{
      error: "access_denied",
      error_description: "The user denied the authorization request"
    })
  end

  defp handle_device_code_status(conn, %{status: "approved", user_id: user_id}) do
    user = Accounts.get_user!(user_id)
    token = Accounts.generate_user_api_token(user)

    json(conn, %{
      data: %{
        access_token: token,
        token_type: "Bearer",
        user: %{id: user.id, email: user.email}
      }
    })
  end

  defp handle_device_code_status(conn, _device_code) do
    conn
    |> put_status(:bad_request)
    |> json(%{error: "invalid_grant", error_description: "The device code is invalid"})
  end

  # Helper to translate changeset errors
  defp translate_errors(changeset) do
    Ecto.Changeset.traverse_errors(changeset, fn {msg, opts} ->
      Regex.replace(~r"%{(\w+)}", msg, fn _, key ->
        opts |> Keyword.get(String.to_existing_atom(key), key) |> to_string()
      end)
    end)
  end
end
