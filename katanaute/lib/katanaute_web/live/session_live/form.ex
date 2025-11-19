defmodule KatanauteWeb.SessionLive.Form do
  use KatanauteWeb, :live_view

  alias Katanaute.Training
  alias Katanaute.Training.Session
  alias Katanaute.Curriculum

  on_mount {KatanauteWeb.Plugs.WebAuth, :ensure_authenticated}

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        {@page_title}
        <:subtitle>Use this form to manage session records in your database.</:subtitle>
      </.header>

      <.form for={@form} id="session-form" phx-change="validate" phx-submit="save">
        <.input
          field={@form[:kata_id]}
          type="select"
          label="Kata"
          options={kata_options(@katas)}
          prompt="Select a kata"
        />
        <.input field={@form[:practiced_at]} type="datetime-local" label="Practiced at" />
        <.input field={@form[:in_course]} type="checkbox" label="In course" />
        <.input field={@form[:notes]} type="textarea" label="Notes" />
        <footer>
          <.button phx-disable-with="Saving..." variant="primary">Save Session</.button>
          <.button navigate={return_path(@return_to, @session)}>Cancel</.button>
        </footer>
      </.form>
    </Layouts.app>
    """
  end

  @impl true
  def mount(params, _session, socket) do
    # Support injecting user_id from params (for testing)
    user_id = params["user_id"]

    {:ok,
     socket
     |> assign(:return_to, return_to(params["return_to"]))
     |> assign(:user_id, user_id)
     |> assign(:katas, Curriculum.list_katas())
     |> apply_action(socket.assigns.live_action, params)}
  end

  defp return_to("show"), do: "show"
  defp return_to(_), do: "index"

  defp apply_action(socket, :edit, %{"id" => id}) do
    user = socket.assigns.current_user
    session = Training.get_user_session!(user.id, id)

    socket
    |> assign(:page_title, "Edit Session")
    |> assign(:session, session)
    |> assign(:form, to_form(Training.change_session(session)))
  end

  defp apply_action(socket, :new, _params) do
    session = %Session{}

    socket
    |> assign(:page_title, "New Session")
    |> assign(:session, session)
    |> assign(:form, to_form(Training.change_session(session)))
  end

  @impl true
  def handle_event("validate", %{"session" => session_params}, socket) do
    changeset = Training.change_session(socket.assigns.session, session_params)
    {:noreply, assign(socket, form: to_form(changeset, action: :validate))}
  end

  def handle_event("save", %{"session" => session_params}, socket) do
    save_session(socket, socket.assigns.live_action, session_params)
  end

  defp save_session(socket, :edit, session_params) do
    case Training.update_session(socket.assigns.session, session_params) do
      {:ok, session} ->
        {:noreply,
         socket
         |> put_flash(:info, "Session updated successfully")
         |> push_navigate(to: return_path(socket.assigns.return_to, session))}

      {:error, %Ecto.Changeset{} = changeset} ->
        {:noreply, assign(socket, form: to_form(changeset))}
    end
  end

  defp save_session(socket, :new, session_params) do
    # Add user_id: prefer current_user (production), fall back to params user_id (testing)
    session_params =
      cond do
        Map.has_key?(socket.assigns, :current_user) && socket.assigns.current_user ->
          Map.put(session_params, "user_id", socket.assigns.current_user.id)

        socket.assigns.user_id ->
          Map.put(session_params, "user_id", socket.assigns.user_id)

        true ->
          session_params
      end

    case Training.create_session(session_params) do
      {:ok, session} ->
        {:noreply,
         socket
         |> put_flash(:info, "Session created successfully")
         |> push_navigate(to: return_path(socket.assigns.return_to, session))}

      {:error, %Ecto.Changeset{} = changeset} ->
        {:noreply, assign(socket, form: to_form(changeset))}
    end
  end

  defp return_path("index", _session), do: ~p"/admin/sessions"
  defp return_path("show", session), do: ~p"/admin/sessions/#{session}"

  defp kata_options(katas) do
    Enum.map(katas, fn kata ->
      {kata.name, kata.id}
    end)
  end
end
