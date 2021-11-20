class CreateFortune < Jennifer::Migration::Base
  def up
    create_table :fortune do |t|
      t.string :message, {:null => false}
    end
  end

  def down
    drop_table :fortune if table_exists? :fortune
  end
end
