import axios from "axios";

const API_URL = process.env.API_URL;

// タスク取得
export const fetchTasks = async (token: string) => {
  try {
    const response = await axios.get(API_URL + "/api/todos", {
      headers: { Authorization: `Bearer ${token}` },
    });
    return response.data;
  } catch (error) {
    throw new Error("Failed to fetch tasks");
  }
};

// タスク追加
export const addTask = async (
  token: string,
  title: string,
  description: string
) => {
  try {
    await axios.post(
      API_URL + "/api/todo",
      { title, description },
      {
        headers: {
          Authorization: `Bearer ${token}`,
          "Content-Type": "application/json",
        },
      }
    );
  } catch (error) {
    throw new Error("Failed to add task");
  }
};

// タスク更新
export const updateTask = async (token: string, updatedTask: any) => {
  try {
    await axios.post(API_URL + "/api/todo", updatedTask, {
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
    });
  } catch (error) {
    throw new Error("Failed to update task");
  }
};

// タスク削除
export const deleteTask = async (token: string, taskId: number) => {
  try {
    await axios.delete(API_URL + "/api/todo", {
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
      data: { id: taskId },
    });
  } catch (error) {
    throw new Error("Failed to delete task");
  }
};

// タスクステータス更新
export const changeTaskStatus = async (token: string, taskId: number) => {
  try {
    await axios.post(
      API_URL + "/api/todo/complete",
      { id: taskId },
      {
        headers: {
          Authorization: `Bearer ${token}`,
          "Content-Type": "application/json",
        },
      }
    );
  } catch (error) {
    throw new Error("Failed to change task status");
  }
};
