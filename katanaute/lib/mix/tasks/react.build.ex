defmodule Mix.Tasks.React.Build do
  @moduledoc """
  Builds the React frontend and copies it to Phoenix static directory.

  ## Usage

      mix react.build

  This task:
  1. Changes to the katareact directory
  2. Runs `npm install` to ensure dependencies are up to date
  3. Runs `npm run build` to build the React app
  4. Copies the built files from `katareact/dist/` to `katanaute/priv/static/react/`
  """

  use Mix.Task

  @shortdoc "Builds the React frontend and copies to Phoenix static directory"

  @impl Mix.Task
  def run(_args) do
    Mix.shell().info("Building React frontend...")

    # Get paths relative to the project root
    project_root = File.cwd!()
    katareact_path = Path.join([project_root, "..", "katareact"])
    static_react_path = Path.join([project_root, "priv", "static", "react"])

    # Ensure katareact directory exists
    unless File.exists?(katareact_path) do
      Mix.raise("katareact directory not found at #{katareact_path}")
    end

    # Step 1: Install dependencies
    Mix.shell().info("Installing npm dependencies...")

    case System.cmd("npm", ["install"], cd: katareact_path, stderr_to_stdout: true) do
      {output, 0} ->
        Mix.shell().info(output)

      {output, _} ->
        Mix.shell().error(output)
        Mix.raise("Failed to install npm dependencies")
    end

    # Step 2: Build React app
    Mix.shell().info("Building React app...")

    case System.cmd("npm", ["run", "build"], cd: katareact_path, stderr_to_stdout: true) do
      {output, 0} ->
        Mix.shell().info(output)

      {output, _} ->
        Mix.shell().error(output)
        Mix.raise("Failed to build React app")
    end

    # Step 3: Copy built files to Phoenix static directory
    Mix.shell().info("Copying built files to #{static_react_path}...")

    dist_path = Path.join(katareact_path, "dist")

    unless File.exists?(dist_path) do
      Mix.raise("Build output directory not found at #{dist_path}")
    end

    # Remove existing react directory if it exists
    if File.exists?(static_react_path) do
      File.rm_rf!(static_react_path)
    end

    # Create the directory
    File.mkdir_p!(static_react_path)

    # Copy all files from dist to static/react
    case File.cp_r(dist_path, static_react_path) do
      {:ok, _files} ->
        Mix.shell().info("Successfully copied React build to #{static_react_path}")

      {:error, reason, file} ->
        Mix.raise("Failed to copy #{file}: #{inspect(reason)}")
    end

    Mix.shell().info("React build complete! 🎉")
  end
end
