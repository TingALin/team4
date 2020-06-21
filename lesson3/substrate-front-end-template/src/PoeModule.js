import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, FormField } from 'semantic-ui-react';
import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';

function Main(props) {
  //通过节点交互获得实例
  const { api } = useSubstrate();
  //accountpair包含交易的账户信息，通过props来获取accountpair
  const { accountPair } = props;

  // The transaction submission status
  //用usestate生成status,发送交易会用到，
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [receiver, setReceiver] = useState('');


  //利用hook来动态加载和更新
  useEffect(() => {
    let unsubscribe;
    //检查poeModule是否更新，因为proofs是map，要通过digest这个key来查询value
    //digest是hash值？
    api.query.poeModule.proofs(digest, (result) => {
      if (result.isNone) {
        setOwner('none');
        setBlockNumber(0);
      } else {
        setOwner(result(0).toString());
        setBlockNumber(result[1].toNumber());
      }
      //这里是promise
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);
    //如果unsubstribe不是null的话就调用函数
    //当我们在清理组件状态时，会调用unsubstribe来解除监听事件
    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);

  const handlefileChosen = (file) => {
    let fileReader = new FileReader();//读取文件
    //
    const bufferToDigest = () => {
      //转成Uint8Array再转成我们的Array
      const content = Array.from(new Uint8Array(fileReader.result))
        //转成16进制格式来表示u8，用2位，不足2位用0补
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('');//通过join变成string

      const hash = blake2AsHex(content, 256);
      setDigest(hash);
    }
    //读取后回调函数
    fileReader.onloadend = bufferToDigest;
    //通过readAsArrayBuffer来读取
    fileReader.readAsArrayBuffer(file);
  }

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence Module</h1>
      <Form>
        <Form.Field>
          <Input type='file' id='file' label='Your File' onChange={(e) => handlefileChosen(e.target.files[0])} />
        </Form.Field>

        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Create Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest],
              paraFields: [true]
            }}
          />
          <TxButton
            accountPair={accountPair}
            label='Revoke Claim'
            setStatus={setStatus}
            type='Signed-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'invokeClaim',
              inputParams: [digest],
              paraFields: [true]
            }}
          />

        </Form.Field>
      </Form>
      <br />
      <Form>
        <FormField>
          <Input type='file' id='file' label='Your File' onChange={(e) => handlefileChosen(e.target.files[0])} />
          <Input
            label='Claim Receiver'
            state='receiver'
            type='string'
            onChange={(_, { value }) => setReceiver(value)}
          />
        </FormField>
        <FormField>
          <TxButton
            accountPair={accountPair}
            label='Transfer Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest, receiver],
              paraFields: [true]
            }}
          />
        </FormField>
      </Form>
      <br />

      <div>{status}</div>
      <div>{`Claim info, owner: ${owner}, Blocknumber: ${blockNumber}`}</div>
    </Grid.Column>
  );
}

export default function PoeModule(props) {
  const { api } = useSubstrate();
  //检查是否存在
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
  //把下面的三元条件表达式删掉，强制显示，看有什么错误
}
