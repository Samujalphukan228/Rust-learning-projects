import mongoose from 'mongoose';

export const connectMongo = async () => {
  await mongoose.connect('mongodb+srv://samujalphukan_db_user:IMbosq8fd700vHni@cluster0.44h9fyt.mongodb.net/notesDB');
  console.log('MongoDB connected');
};
