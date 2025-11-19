defmodule KatanauteWeb.SessionLive.Index do
  use KatanauteWeb, :live_view

  alias Katanaute.Training

  on_mount {KatanauteWeb.Plugs.WebAuth, :ensure_authenticated}

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        Listing Sessions
        <:actions>
          <.button variant="primary" navigate={~p"/admin/sessions/new"}>
            <.icon name="hero-plus" /> New Session
          </.button>
        </:actions>
      </.header>

      <.table
        id="sessions"
        rows={@streams.sessions}
        row_click={fn {_id, session} -> JS.navigate(~p"/admin/sessions/#{session}") end}
      >
        <:col :let={{_id, session}} label="Kata">{session.kata.name}</:col>
        <:col :let={{_id, session}} label="Level">
          {String.capitalize(to_string(session.kata.level))}
        </:col>
        <:col :let={{_id, session}} label="Practiced at">{session.practiced_at}</:col>
        <:col :let={{_id, session}} label="In course">{session.in_course}</:col>
        <:col :let={{_id, session}} label="Notes">{session.notes}</:col>
        <:action :let={{_id, session}}>
          <div class="sr-only">
            <.link navigate={~p"/admin/sessions/#{session}"}>Show</.link>
          </div>
          <.link navigate={~p"/admin/sessions/#{session}/edit"}>Edit</.link>
        </:action>
        <:action :let={{id, session}}>
          <.link
            phx-click={JS.push("delete", value: %{id: session.id}) |> hide("##{id}")}
            data-confirm="Are you sure?"
          >
            Delete
          </.link>
        </:action>
      </.table>
    </Layouts.app>
    """
  end

  @impl true
  def mount(_params, _session, socket) do
    {:ok,
     socket
     |> assign(:page_title, "Listing Sessions")
     |> stream(:sessions, list_sessions(socket.assigns.current_user))}
  end

  @impl true
  def handle_event("delete", %{"id" => id}, socket) do
    user = socket.assigns.current_user
    session = Training.get_user_session!(user.id, id)
    {:ok, _} = Training.delete_session(session)

    {:noreply, stream_delete(socket, :sessions, session)}
  end

  defp list_sessions(user) do
    Training.list_user_sessions(user.id)
  end
end
