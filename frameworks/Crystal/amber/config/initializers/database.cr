require "jennifer"
require "jennifer/adapter/postgres" 
require "jennifer/adapter/mysql" 

APP_ENV = ENV["AMBER_ENV"]? || "development"

Jennifer::Config.configure do |conf|
  conf.from_uri(ENV["DATABASE_URI"]) if ENV.has_key?("DATABASE_URI")
  conf.logger = Log.for("db", :info)
end

Log.setup "db", :info, Log::IOBackend.new(formatter: Jennifer::Adapter::DBFormatter)