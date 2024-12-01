import { FC, useState } from "react";
import { InputProps } from "@/types/type";

const Input: FC<InputProps> = ({ onSubmit }) => {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");

  const handleSubmit = () => {
    if (!title || !description) return;

    onSubmit(title, description);

    setTitle("");
    setDescription("");
  };

  return (
    <div className="flex justify-center items-center gap-2.5">
      <div className="flex space-x-2">
        <input
          type="text"
          className="border-2 border-gray-300 rounded-xl p-2.5 mr-5"
          placeholder="ミッション名"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
        />
        <input
          type="text"
          className="border-2 border-gray-300 rounded-xl p-2.5"
          placeholder="ミッション詳細"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
        />
      </div>
      <button
        className="mt-2 border-none rounded-xl p-2.5 bg-blue-600 text-white"
        onClick={handleSubmit}
      >
        ミッションを追加
      </button>
    </div>
  );
};

export default Input;
