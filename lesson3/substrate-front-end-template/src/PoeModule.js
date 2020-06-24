// React and Semantic UI elements
import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, TextArea, Label } from 'semantic-ui-react';
// Pre-built Substrate front-end utilities for connecting to a node
// and making a transaction
import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
// Polkadot-JS utilities for hashing data.
import { blake2AsHex } from '@polkadot/util-crypto';
// eslint-disable-next-line
function Main(props) {
  // 通过节点交互获得实例
  // Establish an API to talk to our Substrate node.
  const { api } = useSubstrate();
  // accountpair包含交易的账户信息，通过props来获取accountpair
  // Get the 'selected user' from the `AccountSelector` component.
  const { accountPair } = props;

  // The transaction submission status
  // 用usestate生成status,发送交易会用到，
  // React hooks for all the state variables we track.
  // Learn more at: https://reactjs.org/docs/hooks-intro.html
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('none');
  const [blockNumber, setBlockNumber] = useState(0);
  const [receiver, setReceiver] = useState('none');
  const [amount, setAmount] = useState(0);
  const [note, setNote] = useState('');
  const [showingNotification, setShowingNotification] = useState(false);
  useEffect(() => { // 利用hook来动态加载和更新
    let unsubscribe;// React hook to update the 'Owner' and 'Block Number' information for a file.
    // 检查poeModule是否更新，因为proofs是map，要通过digest这个key来查询value
    // digest是hash值？
    // Polkadot-JS API query to the `proofs` storage item in our pallet.
    // This is a subscription, so it will always get the latest value,
    // even if it changes.
    api.query.poeModule.proofs(digest, (result) => {
      if (result.isNone) {
        setOwner('none');
        setBlockNumber(0);
      } else {
        // Our storage item returns a tuple, which is represented as an array.
        setOwner(result[0].toString());
        setBlockNumber(result[1].toNumber());
      }
      // 这里是promise
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);
    // 如果unsubstribe不是null的话就调用函数
    // 当我们在清理组件状态时，会调用unsubstribe来解除监听事件
    return () => unsubscribe && unsubscribe();
    // This tells the React hook to update whenever the file digest changes
    // (when a new file is chosen), or when the storage subscription says the
    // value of the storage item has updated.
  }, [digest, api.query.poeModule]);
  // Callback function for when a new file is selected.
  function handlefileChosen(file) { // const handleFileChosen = (file) => 用这个会有error
    let fileReader = new FileReader();// 读取文件
    // Takes our file, and creates a digest using the Blake2 256 hash function.
    const bufferToDigest = () => { // function bufferToDigest()
      // 转成Uint8Array再转成我们的Array
      // Turns the file content to a hexadecimal representation.
      const content = Array.from(new Uint8Array(fileReader.result))
        // 转成16进制格式来表示u8，用2位，不足2位用0补
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('');// 通过join变成string

      const hash = blake2AsHex(content, 256);
      setDigest(hash);
    };
    // 读取后回调函数
    fileReader.onloadend = bufferToDigest;
    // 通过readAsArrayBuffer来读取
    fileReader.readAsArrayBuffer(file);
  }

  const onDestChange = (_, data) => {
    setDigest(data.value);
  };

  const onAmountChange = (_, data) => {
    setAmount(data.value);
  };

  const MAX_NOTE_LENGTH = 256;
  const onNoteChange = (_, data) => {
    if (data.value && data.value.length > MAX_NOTE_LENGTH) {
      data.value = data.value.substring(0, MAX_NOTE_LENGTH);
    }
    setNote(data.value);
  };

  const setExtrinsicStatus = (data) => {
    console.log(data);
    console.log(data.indexOf('Finalized'));
    if (data.indexOf('Finalized') !== -1) {
      setShowingNotification(true);
      setTimeout(() => setShowingNotification(false), 20000);
    }
    setStatus(data);
  };

  const SuccessNotification = (props) => {
    const { digest, note } = props;
    const notificationStyle = {
      marginTop: 10,
      border: '1px solid green',
      backgroundColor: 'lightgreen',
      color: 'darkgreen',
      borderRadius: 5,
      padding: 10
    };
    return (
      <div style={notificationStyle}>
        You have successfully claimed file with hash <strong>{digest}</strong> with note <strong>"{note}"</strong>.
      </div>
    );
  };

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence Module</h1>
      <br />
      <Form>
        <Form.Field>
          {/* File selector with a callback to `handleFileChosen`. */}
          <Input type='file' id='file' label='Your File'
            onChange={(e) => handlefileChosen(e.target.files[0])}
          />
        </Form.Field>
        <Form.Field>
          <Label>Note</Label>
          <TextArea
            type='text'
            placeholder='Some note (max 256 chars)'
            state='note'
            maxLength={256}
            onChange={onNoteChange}
          />
        </Form.Field>

        <Form.Field>
          <Input
            type='text'
            label='To'
            placeholder='address'
            state='dest'
            onChange={onDestChange}
          />
        </Form.Field>

        <Form.Field>
          <Input
            fluid
            label='Amount'
            type='number'
            state='amount'
            onChange={onAmountChange}
          />
        </Form.Field>

        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Create Claim WithNote'
            setStatus={setExtrinsicStatus} //  setStatus={setStatus}
            type='SIGNED-TX'
            attrs={
              {
                palletRpc: 'poeModule',
                callable: 'createClaimWithNote', // callable: 'createClaim'
                inputParams: [digest, note], // inputParams: [digest]
                paramFields: [true, true] // paramFields: [true]
              }
            }
          />

          <TxButton
            accountPair={accountPair}
            label='Revoke Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'revokeClaim',
              inputParams: [digest],
              paramFields: [true]
            }}
          />
        </Form.Field>
      </Form>
      <br />
      <Form>
        <Form.Field>
          <Input type='file' id='file' label='Your File'
            onChange={(e) => handlefileChosen(e.target.files[0])} />
          <Input
            label='Claim Receiver'
            state='newValue'
            type='string'
            onChange={(_, { value }) => setReceiver(value)}
          />
        </Form.Field>
        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Transfer Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest, receiver],
              paramFields: [true] // paramFields: [true, true]
            }}
          />

          <TxButton
            accountPair={accountPair}
            label='Attach Claim Price'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={
              {
                palletRpc: 'poeModule',
                callable: 'attachClaimPrice',
                inputParams: [digest, amount],
                paramFields: [true, true] // ???
              }
            }
          />

          <TxButton
            accountPair={accountPair}
            label='Buy Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={
              {
                palletRpc: 'poeModule',
                callable: 'buyClaim',
                inputParams: [digest, amount],
                paramFields: [true, true]
              }
            }
          />
        </Form.Field>
      </Form>
      <br />
      {showingNotification && <SuccessNotification digest={digest} note={note} />}
      <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      <div>{`Claim info, owner: ${owner}, Blocknumber: ${blockNumber}`}</div>
    </Grid.Column>
  );
}
// eslint-disable-next-line
export default function PoeModule(props) {
  const { api } = useSubstrate();
  // 检查是否存在
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
  // 把下面的三元条件表达式删掉，强制显示，看有什么错误
}
