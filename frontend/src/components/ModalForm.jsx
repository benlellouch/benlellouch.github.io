import React from 'react'
import './ModalForm.css'


const ModalForm = ({form}) => {
    return(
        <div className='modal'>
            <div className='modal_content'>
                {form}
            </div>
            
        </div>
        

    )

}

export default ModalForm
