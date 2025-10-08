defmodule KatanauteWeb.SessionJSON do
  alias Katanaute.Training.Session
  alias KatanauteWeb.KataJSON

  @doc """
  Renders a list of session.
  """
  def index(%{session: session}) do
    %{data: for(session <- session, do: data(session))}
  end

  @doc """
  Renders a single session.
  """
  def show(%{session: session}) do
    %{data: data(session)}
  end

  defp data(%Session{} = session) do
    kata =
      if Ecto.assoc_loaded?(session.kata),
        do: KataJSON.data(session.kata),
        else: nil

    %{
      id: session.id,
      practiced_at: session.practiced_at,
      in_course: session.in_course,
      notes: session.notes,
      kata: kata,
      kata_id: session.kata_id
    }
  end
end
