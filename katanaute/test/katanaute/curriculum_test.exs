defmodule Katanaute.CurriculumTest do
  use Katanaute.DataCase

  alias Katanaute.Curriculum

  describe "katas" do
    alias Katanaute.Curriculum.Kata

    import Katanaute.CurriculumFixtures

    @invalid_attrs %{name: nil, level: nil}

    test "list_katas/0 returns all katas" do
      kata = kata_fixture()
      assert Curriculum.list_katas() == [kata]
    end

    test "get_kata!/1 returns the kata with given id" do
      kata = kata_fixture()
      assert Curriculum.get_kata!(kata.id) == kata
    end

    test "create_kata/1 with valid data creates a kata" do
      valid_attrs = %{name: "some name", level: :yellow}

      assert {:ok, %Kata{} = kata} = Curriculum.create_kata(valid_attrs)
      assert kata.name == "some name"
      assert kata.level == :yellow
    end

    test "create_kata/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Curriculum.create_kata(@invalid_attrs)
    end

    test "update_kata/2 with valid data updates the kata" do
      kata = kata_fixture()
      update_attrs = %{name: "some updated name", level: :orange}

      assert {:ok, %Kata{} = kata} = Curriculum.update_kata(kata, update_attrs)
      assert kata.name == "some updated name"
      assert kata.level == :orange
    end

    test "update_kata/2 with invalid data returns error changeset" do
      kata = kata_fixture()
      assert {:error, %Ecto.Changeset{}} = Curriculum.update_kata(kata, @invalid_attrs)
      assert kata == Curriculum.get_kata!(kata.id)
    end

    test "delete_kata/1 deletes the kata" do
      kata = kata_fixture()
      assert {:ok, %Kata{}} = Curriculum.delete_kata(kata)
      assert_raise Ecto.NoResultsError, fn -> Curriculum.get_kata!(kata.id) end
    end

    test "change_kata/1 returns a kata changeset" do
      kata = kata_fixture()
      assert %Ecto.Changeset{} = Curriculum.change_kata(kata)
    end
  end
end
