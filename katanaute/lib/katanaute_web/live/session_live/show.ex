defmodule KatanauteWeb.SessionLive.Show do
  use KatanauteWeb, :live_view

  alias Katanaute.Training

  on_mount {KatanauteWeb.Plugs.WebAuth, :ensure_authenticated}

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        Session {@session.id}
        <:subtitle>This is a session record from your database.</:subtitle>
        <:actions>
          <.button navigate={~p"/admin/sessions"}>
            <.icon name="hero-arrow-left" />
          </.button>
          <.button variant="primary" navigate={~p"/admin/sessions/#{@session}/edit?return_to=show"}>
            <.icon name="hero-pencil-square" /> Edit session
          </.button>
        </:actions>
      </.header>

      <.list>
        <:item title="Kata">{@session.kata.name}</:item>
        <:item title="Level">{String.capitalize(to_string(@session.kata.level))}</:item>
        <:item title="Practiced at">{@session.practiced_at}</:item>
        <:item title="In course">{@session.in_course}</:item>
        <:item title="Notes">{@session.notes}</:item>
      </.list>
    </Layouts.app>
    """
  end

  @impl true
  def mount(%{"id" => id}, _session, socket) do
    user = socket.assigns.current_user

    {:ok,
     socket
     |> assign(:page_title, "Show Session")
     |> assign(:session, Training.get_user_session!(user.id, id))}
  end
end
