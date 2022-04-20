import React, { createContext, useState } from "react";
import ChatBar from "./ChatBar";
import Conversation from "./Conversation";

import './styles/output.css'


function App() {
	const [message, newMessage] = useState('')
	const [selectedChat, selectChat] = useState(null)

	return (
		<div className="flex">
			<ChatBar message={message} selectedChat={selectedChat} selectChat={selectChat}/>
			<Conversation newMessage={newMessage} />
		</div>
	)
}

export const MessageContext = createContext(null)
export default App;