import React, { useState } from "react";
import Message from "./Message"
import { v4 as uuidv4 } from 'uuid';

const Conversation = ({ newMessage, chat }) => {
    const [message, setMessage] = useState('')
    const [messages, updateMessages] = useState([])

    const sendMessage = () => {
        if (message.length > 500) {
            console.log('message too long')
            return
        } else if (message.length === 0) {
            console.log('message empty')
            return
        }

        const newMessages = messages
        newMessages.push(<Message key={uuidv4()} us={true} text={message} />)

        updateMessages(newMessages.reverse())
        newMessage(message)
        setMessage('')
    }

    const showChat = () => {
        if (chat === null) {
            return (
                <div className="h-full flex justify-center items-center">
                    <h1 className='text-center'>
                        You are not talking with anyone... <br />
                        Try starting a new chat
                    </h1>
                </div>
            )
        }

        return (
            <>
                <div className='h-full w-full flex flex-col-reverse
                    justify-items-end overflow-x-hidden overflow-auto'>
                    {messages}
                </div>
                <div className="flex mr-2 my-6 justify-end">
                    <input className="mx-4 px-3 w-2/3 py-2 border border-gray-300
                        placeholder-gray-500 text-gray-900 rounded-md focus:outline-none
                        focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                        placeholder="Write something..."
                        value={message}
                        onChange={e => setMessage(e.target.value)}
                        onKeyUp={e => e.key === 'Enter' ? sendMessage() : null}
                    />
                    <img
                        className="ml-2 mr-6 my-1 w-8 h-8 cursor-pointer transition ease-in-out
                        duration-499 hover:scale-110"
                        src={require('./assets/send.png')}
                        onClick={sendMessage}
                    />
                </div>
            </>
        )
    }

    return (
        <div className="w-2/3 h-screen flex flex-col justify-between">
            {showChat()}
        </div>
    )
}

export default Conversation;