defmodule Katanaute.TrainingTest do
  use Katanaute.DataCase

  alias Katanaute.Training

  describe "sessions" do
    alias Katanaute.Training.Session

    import Katanaute.TrainingFixtures

    @invalid_attrs %{practiced_at: nil, in_course: nil, notes: nil}

    test "list_sessions/0 returns all sessions" do
      session = session_fixture()
      assert Training.list_sessions() == [session]
    end

    test "get_session!/1 returns the session with given id" do
      session = session_fixture()
      assert Training.get_session!(session.id) == session
    end

    test "create_session/1 with valid data creates a session" do
      valid_attrs = %{practiced_at: ~U[2025-09-19 00:35:00Z], in_course: true, notes: "some notes"}

      assert {:ok, %Session{} = session} = Training.create_session(valid_attrs)
      assert session.practiced_at == ~U[2025-09-19 00:35:00Z]
      assert session.in_course == true
      assert session.notes == "some notes"
    end

    test "create_session/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Training.create_session(@invalid_attrs)
    end

    test "update_session/2 with valid data updates the session" do
      session = session_fixture()
      update_attrs = %{practiced_at: ~U[2025-09-20 00:35:00Z], in_course: false, notes: "some updated notes"}

      assert {:ok, %Session{} = session} = Training.update_session(session, update_attrs)
      assert session.practiced_at == ~U[2025-09-20 00:35:00Z]
      assert session.in_course == false
      assert session.notes == "some updated notes"
    end

    test "update_session/2 with invalid data returns error changeset" do
      session = session_fixture()
      assert {:error, %Ecto.Changeset{}} = Training.update_session(session, @invalid_attrs)
      assert session == Training.get_session!(session.id)
    end

    test "delete_session/1 deletes the session" do
      session = session_fixture()
      assert {:ok, %Session{}} = Training.delete_session(session)
      assert_raise Ecto.NoResultsError, fn -> Training.get_session!(session.id) end
    end

    test "change_session/1 returns a session changeset" do
      session = session_fixture()
      assert %Ecto.Changeset{} = Training.change_session(session)
    end
  end
end
