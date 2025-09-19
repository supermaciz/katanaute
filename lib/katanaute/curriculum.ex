defmodule Katanaute.Curriculum do
  @moduledoc """
  The Curriculum context.
  """

  import Ecto.Query, warn: false
  alias Katanaute.Repo

  alias Katanaute.Curriculum.Kata

  @doc """
  Returns the list of katas.

  ## Examples

      iex> list_katas()
      [%Kata{}, ...]

  """
  def list_katas do
    Repo.all(Kata)
  end

  @doc """
  Gets a single kata.

  Raises `Ecto.NoResultsError` if the Kata does not exist.

  ## Examples

      iex> get_kata!(123)
      %Kata{}

      iex> get_kata!(456)
      ** (Ecto.NoResultsError)

  """
  def get_kata!(id), do: Repo.get!(Kata, id)

  @doc """
  Creates a kata.

  ## Examples

      iex> create_kata(%{field: value})
      {:ok, %Kata{}}

      iex> create_kata(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_kata(attrs) do
    %Kata{}
    |> Kata.changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Updates a kata.

  ## Examples

      iex> update_kata(kata, %{field: new_value})
      {:ok, %Kata{}}

      iex> update_kata(kata, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_kata(%Kata{} = kata, attrs) do
    kata
    |> Kata.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a kata.

  ## Examples

      iex> delete_kata(kata)
      {:ok, %Kata{}}

      iex> delete_kata(kata)
      {:error, %Ecto.Changeset{}}

  """
  def delete_kata(%Kata{} = kata) do
    Repo.delete(kata)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking kata changes.

  ## Examples

      iex> change_kata(kata)
      %Ecto.Changeset{data: %Kata{}}

  """
  def change_kata(%Kata{} = kata, attrs \\ %{}) do
    Kata.changeset(kata, attrs)
  end
end
