Amber::Server.configure do |app|
  pipeline :web do
    plug Amber::Pipe::Logger.new
  end

  routes :web do
    get "/queries", BenchmarkController, :queries
    get "/updates", BenchmarkController, :updates
    get "/db", BenchmarkController, :db
    get "/json", BenchmarkController, :json
    get "/fortunes", BenchmarkController, :fortunes
    get "/plaintext", BenchmarkController, :plaintext
  end
end
