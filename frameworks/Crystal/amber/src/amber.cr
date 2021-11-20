require "dotenv"
Dotenv.load ".env" if File.file?(".env")

require "../config/*"

Log.setup(:error)
Amber::Server.start
