import React from 'react'
import classNames from 'classnames'

const Message = props => {
    return (
        <div className={classNames({
            'flex': true,
            'justify-end': props.us,
        })}>
            <div className={classNames({
                'bg-purple-500': props.us,
                'w-64 mx-8 my-1 px-4 py-2 bg-gray-200  rounded-md': true
            })}>
                <p className={classNames({
                    'text-white': props.us,
                    'break-words': true
                })}>{props.text}</p>
            </div>
        </div>
    )
}


export default Message;