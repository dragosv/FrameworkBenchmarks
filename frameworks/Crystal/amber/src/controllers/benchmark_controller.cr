class BenchmarkController < Amber::Controller::Base
  LAYOUT     = "main.ecr"
  HTML_UTF8  = "text/html; charset=UTF-8"
  JSON       = "application/json"
  TEXT_PLAIN = "text/plain"
  ID_MAXIMUM = 10_000

  before_action do
    all do
      response.headers["Server"] = "Amber"
      response.headers["Date"] = HTTP.format_time(Time.local)
    end
  end

  def plaintext
    response.content_type = TEXT_PLAIN
    "Hello, World!"
  end

  def json
    response.content_type = JSON
    {message: "Hello, World!"}.to_json
  end

  def db
    response.content_type = JSON
    results = {} of Symbol => Int32
    if world = World.find rand(1..ID_MAXIMUM)
      results = {id: world.id, randomNumber: world.random_number}
    end
    results.to_json
  end

  def queries
    response.content_type = JSON
    queries = params["queries"]
    queries = queries.to_i? || 1
    queries = queries.clamp(1..500)

    results = (1..queries).map do
      if world = World.find rand(1..ID_MAXIMUM)
        {id: world.id, randomNumber: world.random_number}
      end
    end

    results.to_json
  end

  def updates
    response.content_type = JSON
    queries = params["queries"]
    queries = queries.to_i? || 1
    queries = queries.clamp(1..500)

    results = (1..queries).map do
      if world = World.find rand(1..ID_MAXIMUM)

        random_number = rand(1..ID_MAXIMUM)

        world.update({:random_number => random_number})

        {id: world.id, randomNumber: random_number}
      end
    end

    results.to_json
  end

  def fortunes
    response.content_type = HTML_UTF8
    fortune = Fortune.new({:id => 0, :message => "Additional fortune added at request time."})
    # fortune.id = 0
    # fortune.message = 

    fortunes = Fortune.all.to_a
    fortunes << fortune
    fortunes.sort_by! { |fortune| fortune.message || "" }

    render("fortune/index.ecr")
  end
end
