defmodule KatanauteWeb.SessionLive.Show do
  use KatanauteWeb, :live_view

  alias Katanaute.Training

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        Session {@session.id}
        <:subtitle>This is a session record from your database.</:subtitle>
        <:actions>
          <.button navigate={~p"/sessions"}>
            <.icon name="hero-arrow-left" />
          </.button>
          <.button variant="primary" navigate={~p"/sessions/#{@session}/edit?return_to=show"}>
            <.icon name="hero-pencil-square" /> Edit session
          </.button>
        </:actions>
      </.header>

      <.list>
        <:item title="Practiced at">{@session.practiced_at}</:item>
        <:item title="In course">{@session.in_course}</:item>
        <:item title="Notes">{@session.notes}</:item>
      </.list>
    </Layouts.app>
    """
  end

  @impl true
  def mount(%{"id" => id}, _session, socket) do
    {:ok,
     socket
     |> assign(:page_title, "Show Session")
     |> assign(:session, Training.get_session!(id))}
  end
end
