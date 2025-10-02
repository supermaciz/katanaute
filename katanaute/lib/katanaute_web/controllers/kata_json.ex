defmodule KatanauteWeb.KataJSON do
  alias Katanaute.Curriculum.Kata

  @doc """
  Renders a list of katas.
  """
  def index(%{katas: katas}) do
    %{data: for(kata <- katas, do: data(kata))}
  end

  @doc """
  Renders a single kata.
  """
  def show(%{kata: kata}) do
    %{data: data(kata)}
  end

  def data(%Kata{} = kata) do
    %{
      id: kata.id,
      name: kata.name,
      level: kata.level
    }
  end
end
