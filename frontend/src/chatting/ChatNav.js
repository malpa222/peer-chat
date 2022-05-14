import React from 'react'
import { Disclosure } from '@headlessui/react'
import ChatList from './ChatList'

export default function ChatNav() {
    return (
        <Disclosure>
            <Disclosure.Panel static>
                <div className="my-2 mx-4 pb-6">
                    <p className='text-black-300 text-xl my-2'>Chats</p>
                    <input
                        className="border focus:border-indigo-500/75 block w-11/12 pl-7 pr-12 py-2 rounded-md shadow-md"
                        placeholder="Search"
                    />
                    <ChatList />
                </div>
            </Disclosure.Panel>
        </Disclosure>
    )
}