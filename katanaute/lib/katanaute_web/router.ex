defmodule KatanauteWeb.Router do
  use KatanauteWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {KatanauteWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
    plug KatanauteWeb.Plugs.ApiAuth
  end

  pipeline :api_authenticated do
    plug KatanauteWeb.Plugs.ApiAuth, :require_authenticated_user
  end

  # Pipelines for web authentication
  pipeline :fetch_current_user do
    plug KatanauteWeb.Plugs.WebAuth, :fetch_current_user
  end

  pipeline :redirect_if_authenticated do
    plug KatanauteWeb.Plugs.WebAuth, :fetch_current_user
    plug KatanauteWeb.Plugs.WebAuth, :redirect_if_user_is_authenticated
  end

  pipeline :require_authenticated do
    plug KatanauteWeb.Plugs.WebAuth, :fetch_current_user
    plug KatanauteWeb.Plugs.WebAuth, :require_authenticated_user
  end

  # Admin/LiveView routes (moved from root to /admin)
  scope "/admin", KatanauteWeb do
    pipe_through :browser

    get "/", PageController, :home

    # Authenticated LiveView routes
    live_session :authenticated_sessions,
      on_mount: [{KatanauteWeb.Plugs.WebAuth, :ensure_authenticated}] do
      live "/sessions", SessionLive.Index, :index
      live "/sessions/new", SessionLive.Form, :new
      live "/sessions/:id", SessionLive.Show, :show
      live "/sessions/:id/edit", SessionLive.Form, :edit
    end
  end

  ## Authentication routes (admin scope)
  scope "/admin", KatanauteWeb do
    pipe_through [:browser, :redirect_if_authenticated]

    get "/users/register", UserRegistrationController, :new
    post "/users/register", UserRegistrationController, :create
    get "/users/log_in", UserSessionController, :new
    post "/users/log_in", UserSessionController, :create
  end

  scope "/admin", KatanauteWeb do
    pipe_through [:browser, :fetch_current_user]

    delete "/users/log_out", UserSessionController, :delete

    # Device authorization flow (publicly accessible to enter code, but checks if user is logged in)
    get "/device", DeviceController, :new
    post "/device", DeviceController, :verify
  end

  scope "/admin", KatanauteWeb do
    pipe_through [:browser, :require_authenticated]

    # Device authorization (requires authentication)
    get "/device/authorize", DeviceController, :authorize
    post "/device/approve", DeviceController, :approve
    post "/device/deny", DeviceController, :deny
  end

  # Public API routes (no authentication required)
  scope "/api", KatanauteWeb do
    pipe_through :api

    # Authentication endpoints
    post "/auth/register", API.AuthController, :register
    post "/auth/token", API.AuthController, :create_token
    delete "/auth/token", API.AuthController, :delete_token
    post "/auth/device/code", API.AuthController, :device_code
    post "/auth/device/token", API.AuthController, :device_token

    # Public kata list and management
    resources "/katas", KataController
  end

  # Authenticated API routes
  scope "/api", KatanauteWeb do
    pipe_through [:api, :api_authenticated]

    get "/auth/me", API.AuthController, :me
    resources "/sessions", SessionController, except: [:new, :edit]
  end

  # Enable LiveDashboard and Swoosh mailbox preview in development
  if Application.compile_env(:katanaute, :dev_routes) do
    # If you want to use the LiveDashboard in production, you should put
    # it behind authentication and allow only admins to access it.
    # If your application does not have an admins-only section yet,
    # you can use Plug.BasicAuth to set up some basic authentication
    # as long as you are also using SSL (which you should anyway).
    import Phoenix.LiveDashboard.Router

    scope "/dev" do
      pipe_through :browser

      live_dashboard "/dashboard", metrics: KatanauteWeb.Telemetry
      forward "/mailbox", Plug.Swoosh.MailboxPreview
    end
  end

  # React SPA catch-all route (must be last, after API and admin routes)
  scope "/", KatanauteWeb do
    pipe_through :browser

    get "/*path", PageController, :react
  end
end
